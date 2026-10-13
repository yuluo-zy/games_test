// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/text.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5e236f2759a5d08d9c1be47f3e8d36670027b3b247a1366c08d77476b7d0853f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: text.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1185, 1238, 2829, 3953
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use walkdir; // namespace candidate
//   package identities: [{"name":"walkdir","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":2324,"rva":32099920,"va":5400809040}
// 编译实例 RVAs: 0x1e9ce50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18165340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8a4d1eafe","size":5,"soft_fail":false,"target_va":5382644528,"va":5400809863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ttf_parser..language..Language$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-439722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8564af9ff","size":5,"soft_fail":false,"target_va":5400370432,"va":5400810149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$walkdir..IntoIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15635371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8556c11ff","size":5,"soft_fail":false,"target_va":5385174064,"va":5400809430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25956552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e838ef73fe","size":5,"soft_fail":false,"target_va":5374855296,"va":5400811843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25728452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e83c6a77fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400809951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25293655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8a90c7efe","size":5,"soft_fail":false,"target_va":5375516032,"va":5400809682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18104786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e82ebeebfe","size":5,"soft_fail":false,"target_va":5382704752,"va":5400809533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18068427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8354cecfe","size":5,"soft_fail":false,"target_va":5382740736,"va":5400809158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa4080
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15700344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8886e10ff","size":5,"soft_fail":false,"target_va":5385109632,"va":5400809971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb6140
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15625907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e84d9111ff","size":5,"soft_fail":false,"target_va":5385183552,"va":5400809454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428e5af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10781729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e82184a400","size":5,"soft_fail":false,"target_va":5411592944,"va":5400811210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11037674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8ea6ba800","size":5,"soft_fail":false,"target_va":5411847168,"va":5400809489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17970935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e809c9edfe","size":5,"soft_fail":false,"target_va":5382840208,"va":5400811138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18156689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e86ff3eafe","size":5,"soft_fail":false,"target_va":5382655024,"va":5400811708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10731928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e898c1a300","size":5,"soft_fail":false,"target_va":5411542880,"va":5400810947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18065547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e87557ecfe","size":5,"soft_fail":false,"target_va":5382743968,"va":5400809510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::font_faces
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19979883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e89521cffe","size":5,"soft_fail":false,"target_va":5380830112,"va":5400809990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::load_font_from_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19979862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e8aa21cffe","size":5,"soft_fail":false,"target_va":5380830048,"va":5400809905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] walkdir::dent::DirEntry::path
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"walkdir","version":"2.3.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-15544435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400809040,"raw_hex":"e88dcf12ff","size":5,"soft_fail":false,"target_va":5385265072,"va":5400809502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2829,"record_offset":3364,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":12467,"source_line":58,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":3308,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2829,"record_offset":3100,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2829,"record_offset":2572,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2829,"record_offset":3976,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":2692,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":3608,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":7080,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":2792,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2829,"record_offset":3132,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2829,"record_offset":3016,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] walkdir::WalkDir::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2829,"record_offset":2492,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":19900,"source_line":286,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] walkdir::impl$2::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2829,"record_offset":2540,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":19902,"source_line":517,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":2324,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2829,"record_offset":3856,"parent_record_offset":3800,"ancestor_record_offset":2324,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2829,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":2324,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2829,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2324,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":3828,"parent_record_offset":3800,"ancestor_record_offset":2324,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":2324,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":2324,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2829,"record_offset":4588,"parent_record_offset":4532,"ancestor_record_offset":2324,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2829,"record_offset":4496,"parent_record_offset":4464,"ancestor_record_offset":2324,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2829,"record_offset":4532,"parent_record_offset":4496,"ancestor_record_offset":2324,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":2829,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":2324,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2829,"record_offset":3512,"parent_record_offset":3364,"ancestor_record_offset":2324,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":2324,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2829,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2324,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2829,"record_offset":4616,"parent_record_offset":4532,"ancestor_record_offset":2324,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2829,"record_offset":5176,"parent_record_offset":5136,"ancestor_record_offset":2324,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2829,"record_offset":4084,"parent_record_offset":4016,"ancestor_record_offset":2324,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2829,"record_offset":4016,"parent_record_offset":3976,"ancestor_record_offset":2324,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2829,"record_offset":5024,"parent_record_offset":4996,"ancestor_record_offset":2324,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2829,"record_offset":4996,"parent_record_offset":4016,"ancestor_record_offset":2324,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":2829,"record_offset":4204,"parent_record_offset":4172,"ancestor_record_offset":2324,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2829,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":2324,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2829,"record_offset":2648,"parent_record_offset":2620,"ancestor_record_offset":2324,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2829,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":2324,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2829,"record_offset":4172,"parent_record_offset":4084,"ancestor_record_offset":2324,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2829,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":2324,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4140,"parent_record_offset":4084,"ancestor_record_offset":2324,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4936,"parent_record_offset":4084,"ancestor_record_offset":2324,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":2324,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":4392,"parent_record_offset":4204,"ancestor_record_offset":2324,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":2324,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":2829,"record_offset":3188,"parent_record_offset":3156,"ancestor_record_offset":2324,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":2829,"record_offset":3216,"parent_record_offset":3156,"ancestor_record_offset":2324,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":3576,"parent_record_offset":3364,"ancestor_record_offset":2324,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2324,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":2324,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":4276,"parent_record_offset":4240,"ancestor_record_offset":2324,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2829,"record_offset":4644,"parent_record_offset":4532,"ancestor_record_offset":2324,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2829,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":2324,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2829,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":2324,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":2324,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2829,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":2324,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":2324,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":4240,"parent_record_offset":4204,"ancestor_record_offset":2324,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":4304,"parent_record_offset":4240,"ancestor_record_offset":2324,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2829,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2324,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2829,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2324,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2829,"record_offset":3244,"parent_record_offset":3156,"ancestor_record_offset":2324,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2829,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2324,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2829,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":2324,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2829,"record_offset":5412,"parent_record_offset":5136,"ancestor_record_offset":2324,"inlinee":30671,"source_line":52,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2829,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":2324,"inlinee":16035,"source_line":48,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$12::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":2829,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":2324,"inlinee":30670,"source_line":1008,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2829,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2324,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2829,"record_offset":5524,"parent_record_offset":5500,"ancestor_record_offset":2324,"inlinee":61895,"source_line":141,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2829,"record_offset":4464,"parent_record_offset":4428,"ancestor_record_offset":2324,"inlinee":61893,"source_line":144,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2829,"record_offset":2724,"parent_record_offset":2692,"ancestor_record_offset":2324,"inlinee":100389,"source_line":20,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2829,"record_offset":3400,"parent_record_offset":3364,"ancestor_record_offset":2324,"inlinee":100390,"source_line":24,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF3C9
// 首条位置: {"module":1238,"inlinee":62409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1238,"record_offset":60184,"parent_record_offset":60152,"ancestor_record_offset":60152,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1238,"record_offset":60564,"parent_record_offset":60540,"ancestor_record_offset":60152,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1238,"record_offset":60540,"parent_record_offset":60516,"ancestor_record_offset":60152,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1238,"record_offset":60952,"parent_record_offset":60924,"ancestor_record_offset":60152,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1238,"record_offset":61888,"parent_record_offset":61860,"ancestor_record_offset":60152,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1238,"record_offset":60516,"parent_record_offset":60492,"ancestor_record_offset":60152,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1238,"record_offset":60492,"parent_record_offset":60468,"ancestor_record_offset":60152,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1238,"record_offset":60980,"parent_record_offset":60952,"ancestor_record_offset":60152,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1238,"record_offset":61004,"parent_record_offset":60980,"ancestor_record_offset":60152,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1238,"record_offset":60228,"parent_record_offset":60184,"ancestor_record_offset":60152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1238,"record_offset":60752,"parent_record_offset":60720,"ancestor_record_offset":60152,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1238,"record_offset":60800,"parent_record_offset":60776,"ancestor_record_offset":60152,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1238,"record_offset":60776,"parent_record_offset":60752,"ancestor_record_offset":60152,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1238,"record_offset":60720,"parent_record_offset":60688,"ancestor_record_offset":60152,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1238,"record_offset":60688,"parent_record_offset":60660,"ancestor_record_offset":60152,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01238.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18825
// 首条位置: {"module":2829,"inlinee":100389}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2829,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18826
// 首条位置: {"module":2829,"inlinee":100390}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2829,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":3400,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2829,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":3400,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":3400,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::update_text_renderer_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":7740,"rva":32102752,"va":5400811872}
// 编译实例 RVAs: 0x1e9d960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10728935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e8e7b5a300","size":5,"soft_fail":false,"target_va":5411541584,"va":5400812644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e884f0ffff","size":5,"soft_fail":false,"target_va":5400807968,"va":5400811927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10731351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e857bfa300","size":5,"soft_fail":false,"target_va":5411543360,"va":5400812004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19426906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e8a691d7fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5400812437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19421720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e8e8a5d7fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5400812355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19426077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e8e394d7fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5400812552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-19427206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e87a90d7fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5400811905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10782241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e82186a400","size":5,"soft_fail":false,"target_va":5411594832,"va":5400812586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15524029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e8431f13ff","size":5,"soft_fail":false,"target_va":5385288736,"va":5400812760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10724467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e873a4a300","size":5,"soft_fail":false,"target_va":5411537104,"va":5400812632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::set_pixels_per_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19982309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400811872,"raw_hex":"e81b18cffe","size":5,"soft_fail":false,"target_va":5380830192,"va":5400812496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":10284,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":10344,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9316,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":7884,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":8340,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":10312,"parent_record_offset":10284,"ancestor_record_offset":7740,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9636,"parent_record_offset":9608,"ancestor_record_offset":7740,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":7740,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9664,"parent_record_offset":9636,"ancestor_record_offset":7740,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":7740,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":7740,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9780,"parent_record_offset":9608,"ancestor_record_offset":7740,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2829,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":7740,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2829,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":7740,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2829,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7740,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2829,"record_offset":9452,"parent_record_offset":9412,"ancestor_record_offset":7740,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2829,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":7740,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2829,"record_offset":10240,"parent_record_offset":9412,"ancestor_record_offset":7740,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":9844,"parent_record_offset":9556,"ancestor_record_offset":7740,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":7740,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":7740,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":7740,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2829,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":7740,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":7740,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":7740,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":9008,"parent_record_offset":8980,"ancestor_record_offset":7740,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":8656,"parent_record_offset":8540,"ancestor_record_offset":7740,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7740,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2829,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":7740,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":8508,"parent_record_offset":8476,"ancestor_record_offset":7740,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2829,"record_offset":8476,"parent_record_offset":8440,"ancestor_record_offset":7740,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7740,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":7740,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":7740,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2829,"record_offset":8980,"parent_record_offset":8440,"ancestor_record_offset":7740,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7980,"parent_record_offset":7948,"ancestor_record_offset":7740,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":7740,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":7740,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":8236,"parent_record_offset":8140,"ancestor_record_offset":7740,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":7740,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":7920,"parent_record_offset":7884,"ancestor_record_offset":7740,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7740,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8540,"parent_record_offset":8508,"ancestor_record_offset":7740,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8824,"parent_record_offset":8540,"ancestor_record_offset":7740,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8796,"parent_record_offset":8540,"ancestor_record_offset":7740,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2829,"record_offset":8600,"parent_record_offset":8540,"ancestor_record_offset":7740,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":7740,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":10408,"parent_record_offset":10376,"ancestor_record_offset":7740,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":10908,"parent_record_offset":10648,"ancestor_record_offset":7740,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2829,"record_offset":9480,"parent_record_offset":9452,"ancestor_record_offset":7740,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9556,"parent_record_offset":9412,"ancestor_record_offset":7740,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2829,"record_offset":9816,"parent_record_offset":9556,"ancestor_record_offset":7740,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2829,"record_offset":9508,"parent_record_offset":9480,"ancestor_record_offset":7740,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":9412,"parent_record_offset":9380,"ancestor_record_offset":7740,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2829,"record_offset":9380,"parent_record_offset":9348,"ancestor_record_offset":7740,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::prepare_text_fragments
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":11500,"rva":32103712,"va":5400812832}
// 编译实例 RVAs: 0x1e9dd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::prepare
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19975888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400812832,"raw_hex":"e83031cffe","size":5,"soft_fail":false,"target_va":5380837216,"va":5400813099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":14332,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":14268,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":12040,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":11636,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":11500,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":11500,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":11500,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":11500,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11824,"parent_record_offset":11792,"ancestor_record_offset":11500,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":11500,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11500,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11500,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":13868,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":13200,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":13284,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":14064,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":13116,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":14360,"parent_record_offset":14332,"ancestor_record_offset":11500,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":14296,"parent_record_offset":14268,"ancestor_record_offset":11500,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":11500,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":13376,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12428,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12892,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":11500,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":11744,"parent_record_offset":11660,"ancestor_record_offset":11500,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":11500,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":12088,"parent_record_offset":12040,"ancestor_record_offset":11500,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2829,"record_offset":12800,"parent_record_offset":12776,"ancestor_record_offset":11500,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12508,"parent_record_offset":12484,"ancestor_record_offset":11500,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12484,"parent_record_offset":12460,"ancestor_record_offset":11500,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":11500,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":12460,"parent_record_offset":12428,"ancestor_record_offset":11500,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":12344,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":11500,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":12260,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2829,"record_offset":12832,"parent_record_offset":12428,"ancestor_record_offset":11500,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2829,"record_offset":12776,"parent_record_offset":12428,"ancestor_record_offset":11500,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":2829,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":11500,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":11500,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":11500,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":11688,"parent_record_offset":11660,"ancestor_record_offset":11500,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2829,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":11500,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2829,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":11500,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":11500,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":11500,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":12620,"parent_record_offset":12484,"ancestor_record_offset":11500,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":11500,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":12224,"parent_record_offset":12188,"ancestor_record_offset":11500,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":12284,"parent_record_offset":12260,"ancestor_record_offset":11500,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":11500,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11500,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":12700,"parent_record_offset":12668,"ancestor_record_offset":11500,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2829,"record_offset":11964,"parent_record_offset":11792,"ancestor_record_offset":11500,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":12188,"parent_record_offset":12088,"ancestor_record_offset":11500,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2829,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":11500,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::render_text_fragments
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":14400,"rva":32104288,"va":5400813408}
// 编译实例 RVAs: 0x1e9df60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19975940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400813408,"raw_hex":"e8fc30cffe","size":5,"soft_fail":false,"target_va":5380837664,"va":5400813599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18180,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18116,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":14936,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":14532,"parent_record_offset":14400,"ancestor_record_offset":14400,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":14768,"parent_record_offset":14744,"ancestor_record_offset":14400,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":14400,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2829,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":14400,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":14400,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":14720,"parent_record_offset":14688,"ancestor_record_offset":14400,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":14400,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":14400,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2829,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":14400,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":16580,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":17304,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":17388,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":16776,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2829,"record_offset":17220,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18208,"parent_record_offset":18180,"ancestor_record_offset":14400,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18144,"parent_record_offset":18116,"ancestor_record_offset":14400,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":14400,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17616,"parent_record_offset":17480,"ancestor_record_offset":14400,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":16116,"parent_record_offset":15628,"ancestor_record_offset":14400,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15244,"parent_record_offset":15068,"ancestor_record_offset":14400,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15292,"parent_record_offset":15268,"ancestor_record_offset":14400,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":14400,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":14400,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15096,"parent_record_offset":15068,"ancestor_record_offset":14400,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15144,"parent_record_offset":15120,"ancestor_record_offset":14400,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2829,"record_offset":15068,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":14640,"parent_record_offset":14556,"ancestor_record_offset":14400,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":14400,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2829,"record_offset":14980,"parent_record_offset":14936,"ancestor_record_offset":14400,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":17480,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2829,"record_offset":15628,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2829,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":14400,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":14400,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":14400,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2829,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":14400,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":15684,"parent_record_offset":15652,"ancestor_record_offset":14400,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":14400,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":15544,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":14400,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":15460,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2829,"record_offset":16056,"parent_record_offset":15652,"ancestor_record_offset":14400,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2829,"record_offset":16000,"parent_record_offset":15652,"ancestor_record_offset":14400,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2829,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":14400,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":2829,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":14400,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":14608,"parent_record_offset":14584,"ancestor_record_offset":14400,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14400,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2829,"record_offset":14584,"parent_record_offset":14556,"ancestor_record_offset":14400,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2829,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":14400,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2829,"record_offset":15780,"parent_record_offset":15756,"ancestor_record_offset":14400,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":15756,"parent_record_offset":15732,"ancestor_record_offset":14400,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":15804,"parent_record_offset":15780,"ancestor_record_offset":14400,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":15844,"parent_record_offset":15708,"ancestor_record_offset":14400,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2829,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14400,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2829,"record_offset":15424,"parent_record_offset":15388,"ancestor_record_offset":14400,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2829,"record_offset":15484,"parent_record_offset":15460,"ancestor_record_offset":14400,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":14400,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":15508,"parent_record_offset":15484,"ancestor_record_offset":14400,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2829,"record_offset":15924,"parent_record_offset":15892,"ancestor_record_offset":14400,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2829,"record_offset":14860,"parent_record_offset":14688,"ancestor_record_offset":14400,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2829,"record_offset":15388,"parent_record_offset":14980,"ancestor_record_offset":14400,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2829,"record_offset":15892,"parent_record_offset":15868,"ancestor_record_offset":14400,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::flush_text_render
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2829,"record_offset":18248,"rva":32104944,"va":5400814064}
// 编译实例 RVAs: 0x1e9e1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22111828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e8ac99aefe","size":5,"soft_fail":false,"target_va":5378702496,"va":5400814319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15670019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e8fde410ff","size":5,"soft_fail":false,"target_va":5385144336,"va":5400814350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423a3280
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5263471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e86f505000","size":5,"soft_fail":false,"target_va":5406077568,"va":5400814092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11032989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e89d59a800","size":5,"soft_fail":false,"target_va":5411847152,"va":5400814158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10727308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e88cafa300","size":5,"soft_fail":false,"target_va":5411541776,"va":5400814463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10668022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400814064,"raw_hex":"e8f6c7a200","size":5,"soft_fail":false,"target_va":5411482433,"va":5400814406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5400814064,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5400814178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18916,"parent_record_offset":18248,"ancestor_record_offset":18248,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2829,"record_offset":18608,"parent_record_offset":18248,"ancestor_record_offset":18248,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2829,"record_offset":18372,"parent_record_offset":18248,"ancestor_record_offset":18248,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2829,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":18248,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2829,"record_offset":18440,"parent_record_offset":18400,"ancestor_record_offset":18248,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2829,"record_offset":18524,"parent_record_offset":18496,"ancestor_record_offset":18248,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2829,"record_offset":18552,"parent_record_offset":18524,"ancestor_record_offset":18248,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2829,"record_offset":18468,"parent_record_offset":18440,"ancestor_record_offset":18248,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2829,"record_offset":18400,"parent_record_offset":18372,"ancestor_record_offset":18248,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/02829.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::flush_text_render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1CE
// 首条位置: {"module":1185,"inlinee":61902}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1185,"record_offset":7504,"parent_record_offset":7040,"ancestor_record_offset":7040,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1185,"record_offset":7096,"parent_record_offset":7040,"ancestor_record_offset":7040,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1185,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":7040,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1185,"record_offset":7256,"parent_record_offset":7124,"ancestor_record_offset":7040,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1185,"record_offset":7172,"parent_record_offset":7124,"ancestor_record_offset":7040,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1185,"record_offset":7124,"parent_record_offset":7096,"ancestor_record_offset":7040,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/01185.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1185,"record_offset":7200,"parent_record_offset":7124,"ancestor_record_offset":7040,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/01185.jsonl.gz"}
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
// 完整结构及原始类型记录: text.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: text.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: text.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::text::flush_text_render
// 信息 ID: dbb3bf96801228685426b2c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8899861bcffb74c21d1ad7c3: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0xfecaee -> 0x1e9e1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > @ 0x1e0f799 -> 0x1e9e1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::flush_text_render::closure$0
// 信息 ID: 33451219320d7a76d55d0760
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::flush_text_render", "captures": "unknown"}
// [闭包环境类型引用] toolbar::text::flush_text_render::closure_env$0 <- tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; module=1185, inline_record=7040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::prepare_text_fragments
// 信息 ID: 982bb45207155801704ead71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8e3d842129617e04da1c3ac2: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> > @ 0xfec726 -> 0x1e9dd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> > > @ 0x1e091dc -> 0x1e9dd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::render_text_fragments
// 信息 ID: cc8f4b48d990170a63e0e1f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e826ead3593b9199e32767d: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> > @ 0xfec8a6 -> 0x1e9df60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> > > @ 0x1e1518c -> 0x1e9df60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system
// 信息 ID: ee8396c722d77653765bc777
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1a12d802edafed7f79fb1a9f: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> > @ 0xfec5a0 -> 0x1e9ce50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> > > @ 0x1e18f0e -> 0x1e9ce50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$0
// 信息 ID: 37f07891a814b56fcdeb12d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// [闭包环境类型引用] toolbar::text::text_startup_system::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<enum2$<core::result::Result<walkdir::dent::DirEntry,walkdir::error::Error> > >,toolbar::text::text_startup_system::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<enum2$<core::result::Result<walkdir::dent::DirEntry,walkdir::error::Error> > >,toolbar::text::text_startup_system::closure_env$0>; module=1238, inline_record=60152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$1
// 信息 ID: 03567d90872c9b9f71adfb5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2829, inline_record=2724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$2
// 信息 ID: c4f741a10b083b54513910c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2829, inline_record=3400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::update_text_renderer_scale
// 信息 ID: 33174dcc4c915ee3d0d318ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b102c492741a390ab7ddbd1e: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > @ 0xfec3be -> 0x1e9d960 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > > @ 0x1e0b160 -> 0x1e9d960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
