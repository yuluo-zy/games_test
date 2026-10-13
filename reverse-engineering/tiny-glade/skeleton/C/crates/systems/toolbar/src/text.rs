// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/text.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 0801d886995cb2f16e9186d6eee61eb021bb2c7cdb9dccdc27929138d457cb1b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: text.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1240, 1287, 1307, 2942
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
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
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use walkdir; // namespace candidate
//   package identities: [{"name":"walkdir","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.5.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2942,"record_offset":4160,"rva":37365040,"va":5406074160}
// 编译实例 RVAs: 0x23a2530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21417723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e80531b9fe","size":5,"soft_fail":false,"target_va":5384656912,"va":5406074630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21420549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e8fb25b9fe","size":5,"soft_fail":false,"target_va":5384653728,"va":5406074272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21465549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e83376b8fe","size":5,"soft_fail":false,"target_va":5384609408,"va":5406074952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ttf_parser..language..Language$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7093514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e8f6c293ff","size":5,"soft_fail":false,"target_va":5398981840,"va":5406075349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$walkdir..IntoIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18726100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e82c43e2fe","size":5,"soft_fail":false,"target_va":5387348384,"va":5406074479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-30395871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e8213230fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5406074570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36794248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e87890cefd","size":5,"soft_fail":false,"target_va":5369282736,"va":5406076979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36735624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e87875cffd","size":5,"soft_fail":false,"target_va":5369338848,"va":5406074467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141182ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19004052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e86c05defe","size":5,"soft_fail":false,"target_va":5387071136,"va":5406075183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10982544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e89094a700","size":5,"soft_fail":false,"target_va":5417057056,"va":5406074507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21281331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e8cd45bbfe","size":5,"soft_fail":false,"target_va":5384794864,"va":5406076190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h08fb33cb2e750044 (.llvm.3142044339675947671)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10578032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e87068a100","size":5,"soft_fail":false,"target_va":5416654288,"va":5406076251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-21452769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e81fa8b8fe","size":5,"soft_fail":false,"target_va":5384624080,"va":5406076844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10545572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e8a4e9a000","size":5,"soft_fail":false,"target_va":5416621632,"va":5406076055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21244579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e85dd5bbfe","size":5,"soft_fail":false,"target_va":5384830080,"va":5406074654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23155341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e873ad9efe","size":5,"soft_fail":false,"target_va":5382919648,"va":5406074984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::font_faces
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24060135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e819df90fe","size":5,"soft_fail":false,"target_va":5382015072,"va":5406075202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::load_font_from_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24059615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406074160,"raw_hex":"e821e190fe","size":5,"soft_fail":false,"target_va":5382015408,"va":5406075018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":2942,"record_offset":6612,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":6556,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2942,"record_offset":6388,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2942,"record_offset":4408,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2942,"record_offset":7700,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":5916,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2942,"record_offset":6856,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2942,"record_offset":10608,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":6080,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2942,"record_offset":6444,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2942,"record_offset":6304,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] walkdir::WalkDir::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/walkdir-2.5.0/src/lib.rs
//     evidence={"module":2942,"record_offset":4328,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":21972,"source_line":289,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] walkdir::impl$2::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/walkdir-2.5.0/src/lib.rs
//     evidence={"module":2942,"record_offset":4376,"parent_record_offset":4160,"ancestor_record_offset":4160,"inlinee":21974,"source_line":540,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4160,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4160,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":4160,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":4992,"parent_record_offset":4936,"ancestor_record_offset":4160,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2942,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":4160,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2942,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":4160,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":4160,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4160,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4160,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2942,"record_offset":8216,"parent_record_offset":8160,"ancestor_record_offset":4160,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2942,"record_offset":8120,"parent_record_offset":8084,"ancestor_record_offset":4160,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2942,"record_offset":8160,"parent_record_offset":8120,"ancestor_record_offset":4160,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":2942,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":4160,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":2942,"record_offset":6760,"parent_record_offset":6612,"ancestor_record_offset":4160,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":4160,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2942,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":4160,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":2942,"record_offset":8244,"parent_record_offset":8160,"ancestor_record_offset":4160,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2942,"record_offset":8832,"parent_record_offset":8804,"ancestor_record_offset":4160,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2942,"record_offset":7804,"parent_record_offset":7740,"ancestor_record_offset":4160,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":2942,"record_offset":7740,"parent_record_offset":7700,"ancestor_record_offset":4160,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2942,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":4160,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":2942,"record_offset":8680,"parent_record_offset":7740,"ancestor_record_offset":4160,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2942,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":4160,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2942,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":4160,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2942,"record_offset":4508,"parent_record_offset":4472,"ancestor_record_offset":4160,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2942,"record_offset":4472,"parent_record_offset":4440,"ancestor_record_offset":4160,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":7880,"parent_record_offset":7804,"ancestor_record_offset":4160,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":4160,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":8044,"parent_record_offset":8004,"ancestor_record_offset":4160,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":7852,"parent_record_offset":7804,"ancestor_record_offset":4160,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":8620,"parent_record_offset":7804,"ancestor_record_offset":4160,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":4160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":4160,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":8004,"parent_record_offset":7912,"ancestor_record_offset":4160,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":4160,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2942,"record_offset":4648,"parent_record_offset":4604,"ancestor_record_offset":4160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2942,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":4160,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":8912,"parent_record_offset":8856,"ancestor_record_offset":4160,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":4160,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2942,"record_offset":8272,"parent_record_offset":8160,"ancestor_record_offset":4160,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2942,"record_offset":4604,"parent_record_offset":4572,"ancestor_record_offset":4160,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2942,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":4160,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2942,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":4160,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":4160,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2942,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":4160,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":7948,"parent_record_offset":7912,"ancestor_record_offset":4160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2942,"record_offset":6328,"parent_record_offset":6304,"ancestor_record_offset":4160,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":2942,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":4160,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":2942,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":4160,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2942,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":4160,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":2942,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":4160,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":2942,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":4160,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":2942,"record_offset":8948,"parent_record_offset":8804,"ancestor_record_offset":4160,"inlinee":36827,"source_line":48,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":2942,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":4160,"inlinee":4742,"source_line":44,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$16::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":2942,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":4160,"inlinee":36826,"source_line":1169,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":2942,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":4160,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2942,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":4160,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2942,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":4160,"inlinee":112264,"source_line":150,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::impl$2::font_faces::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/lib.rs
//     evidence={"module":2942,"record_offset":8084,"parent_record_offset":8044,"ancestor_record_offset":4160,"inlinee":112262,"source_line":153,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2942,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":4160,"inlinee":70914,"source_line":18,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2942,"record_offset":5948,"parent_record_offset":5916,"ancestor_record_offset":4160,"inlinee":112260,"source_line":20,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::text::text_startup_system::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/text.rs
//     evidence={"module":2942,"record_offset":6648,"parent_record_offset":6612,"ancestor_record_offset":4160,"inlinee":71031,"source_line":24,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11502
// 首条位置: {"module":1287,"inlinee":70914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2942,"record_offset":4604,"parent_record_offset":4572,"ancestor_record_offset":4572,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4572,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4572,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":4572,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2942,"record_offset":4992,"parent_record_offset":4936,"ancestor_record_offset":4572,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":4572,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4572,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4572,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2942,"record_offset":4648,"parent_record_offset":4604,"ancestor_record_offset":4572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B684
// 首条位置: {"module":2942,"inlinee":112260}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":2942,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":5948,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":2942,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":5948,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2942,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":5948,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::text_startup_system::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11577
// 首条位置: {"module":1307,"inlinee":71031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2942,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":6648,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":2942,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":6648,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":6648,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/02942.jsonl.gz"}
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
// 首条位置: {"module":2942,"record_offset":18260,"rva":37369088,"va":5406078208}
// 编译实例 RVAs: 0x23a3500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10542645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e835dea000","size":5,"soft_fail":false,"target_va":5416621408,"va":5406078758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e810ebffff","size":5,"soft_fail":false,"target_va":5406072912,"va":5406078267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10542530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e8c2dda000","size":5,"soft_fail":false,"target_va":5416621376,"va":5406078841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22114829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e8f38daefe","size":5,"soft_fail":false,"target_va":5383963792,"va":5406078616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22106325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e82bafaefe","size":5,"soft_fail":false,"target_va":5383972224,"va":5406078544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22113816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e8e891aefe","size":5,"soft_fail":false,"target_va":5383964912,"va":5406078723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22115430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e89a8baefe","size":5,"soft_fail":false,"target_va":5383962816,"va":5406078241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10577787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e87b67a100","size":5,"soft_fail":false,"target_va":5416656560,"va":5406078768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18747965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e8c3ede1fe","size":5,"soft_fail":false,"target_va":5387331072,"va":5406079032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10540549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e805d6a000","size":5,"soft_fail":false,"target_va":5416619360,"va":5406078806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::set_pixels_per_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24063209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406078208,"raw_hex":"e817d390fe","size":5,"soft_fail":false,"target_va":5382015472,"va":5406078676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":20604,"parent_record_offset":18260,"ancestor_record_offset":18260,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2942,"record_offset":20664,"parent_record_offset":18260,"ancestor_record_offset":18260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":19796,"parent_record_offset":18260,"ancestor_record_offset":18260,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":18404,"parent_record_offset":18260,"ancestor_record_offset":18260,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":18784,"parent_record_offset":18260,"ancestor_record_offset":18260,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":20632,"parent_record_offset":20604,"ancestor_record_offset":18260,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20124,"parent_record_offset":20096,"ancestor_record_offset":18260,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20152,"parent_record_offset":20124,"ancestor_record_offset":18260,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20072,"parent_record_offset":20044,"ancestor_record_offset":18260,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":18260,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20212,"parent_record_offset":20096,"ancestor_record_offset":18260,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2942,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":18260,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2942,"record_offset":20176,"parent_record_offset":20152,"ancestor_record_offset":18260,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":19944,"parent_record_offset":19904,"ancestor_record_offset":18260,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2942,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":18260,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2942,"record_offset":20560,"parent_record_offset":19904,"ancestor_record_offset":18260,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2942,"record_offset":20276,"parent_record_offset":20044,"ancestor_record_offset":18260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":19204,"parent_record_offset":19180,"ancestor_record_offset":18260,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":19180,"parent_record_offset":19156,"ancestor_record_offset":18260,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":19388,"parent_record_offset":19360,"ancestor_record_offset":18260,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2942,"record_offset":19488,"parent_record_offset":19460,"ancestor_record_offset":18260,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":19460,"parent_record_offset":19432,"ancestor_record_offset":18260,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":18260,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":19156,"parent_record_offset":19132,"ancestor_record_offset":18260,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":19516,"parent_record_offset":19488,"ancestor_record_offset":18260,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":19108,"parent_record_offset":18992,"ancestor_record_offset":18260,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2942,"record_offset":18532,"parent_record_offset":18508,"ancestor_record_offset":18260,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2942,"record_offset":18820,"parent_record_offset":18784,"ancestor_record_offset":18260,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2942,"record_offset":18960,"parent_record_offset":18928,"ancestor_record_offset":18260,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2942,"record_offset":18928,"parent_record_offset":18892,"ancestor_record_offset":18260,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2942,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":18260,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2942,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18260,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2942,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":18260,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2942,"record_offset":19432,"parent_record_offset":18892,"ancestor_record_offset":18260,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18508,"parent_record_offset":18476,"ancestor_record_offset":18260,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18476,"parent_record_offset":18448,"ancestor_record_offset":18260,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18632,"parent_record_offset":18600,"ancestor_record_offset":18260,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18684,"parent_record_offset":18632,"ancestor_record_offset":18260,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18600,"parent_record_offset":18572,"ancestor_record_offset":18260,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18448,"parent_record_offset":18404,"ancestor_record_offset":18260,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2942,"record_offset":18708,"parent_record_offset":18684,"ancestor_record_offset":18260,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2942,"record_offset":18992,"parent_record_offset":18960,"ancestor_record_offset":18260,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2942,"record_offset":19276,"parent_record_offset":18992,"ancestor_record_offset":18260,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2942,"record_offset":19248,"parent_record_offset":18992,"ancestor_record_offset":18260,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2942,"record_offset":19052,"parent_record_offset":18992,"ancestor_record_offset":18260,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":19832,"parent_record_offset":19796,"ancestor_record_offset":18260,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":20728,"parent_record_offset":20696,"ancestor_record_offset":18260,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":21168,"parent_record_offset":20964,"ancestor_record_offset":18260,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2942,"record_offset":19972,"parent_record_offset":19944,"ancestor_record_offset":18260,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":20044,"parent_record_offset":19904,"ancestor_record_offset":18260,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2942,"record_offset":20248,"parent_record_offset":20044,"ancestor_record_offset":18260,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2942,"record_offset":20000,"parent_record_offset":19972,"ancestor_record_offset":18260,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2942,"record_offset":19904,"parent_record_offset":19868,"ancestor_record_offset":18260,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2942,"record_offset":19868,"parent_record_offset":19832,"ancestor_record_offset":18260,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02942.jsonl.gz"}
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
// 首条位置: {"module":2942,"record_offset":15364,"rva":37368576,"va":5406077696}
// 编译实例 RVAs: 0x23a3300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::prepare
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24063406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406077696,"raw_hex":"e852d290fe","size":5,"soft_fail":false,"target_va":5382014624,"va":5406078025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":18192,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":18128,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":15932,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":15500,"parent_record_offset":15364,"ancestor_record_offset":15364,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":15364,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":15812,"parent_record_offset":15788,"ancestor_record_offset":15364,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":15788,"parent_record_offset":15764,"ancestor_record_offset":15364,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":15740,"parent_record_offset":15716,"ancestor_record_offset":15364,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":15716,"parent_record_offset":15684,"ancestor_record_offset":15364,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15364,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":15364,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":15684,"parent_record_offset":15660,"ancestor_record_offset":15364,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":16856,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":16216,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":16300,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":17108,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":16132,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":18220,"parent_record_offset":18192,"ancestor_record_offset":15364,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":15364,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":15364,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":16392,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":16416,"parent_record_offset":16392,"ancestor_record_offset":15364,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":17136,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":17184,"parent_record_offset":17160,"ancestor_record_offset":15364,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":15636,"parent_record_offset":15524,"ancestor_record_offset":15364,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":15660,"parent_record_offset":15636,"ancestor_record_offset":15364,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":15964,"parent_record_offset":15932,"ancestor_record_offset":15364,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2942,"record_offset":16760,"parent_record_offset":16736,"ancestor_record_offset":15364,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":16496,"parent_record_offset":16472,"ancestor_record_offset":15364,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":16472,"parent_record_offset":16448,"ancestor_record_offset":15364,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":15364,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":16448,"parent_record_offset":16416,"ancestor_record_offset":15364,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":18036,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":17208,"parent_record_offset":17184,"ancestor_record_offset":15364,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":17952,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2942,"record_offset":16792,"parent_record_offset":16416,"ancestor_record_offset":15364,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2942,"record_offset":16736,"parent_record_offset":16416,"ancestor_record_offset":15364,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":2942,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":15364,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":15576,"parent_record_offset":15552,"ancestor_record_offset":15364,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":15600,"parent_record_offset":15576,"ancestor_record_offset":15364,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":15364,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":15552,"parent_record_offset":15524,"ancestor_record_offset":15364,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2942,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":15364,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":15364,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":15364,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":16568,"parent_record_offset":16544,"ancestor_record_offset":15364,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":16608,"parent_record_offset":16472,"ancestor_record_offset":15364,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2942,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":15364,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":16096,"parent_record_offset":16068,"ancestor_record_offset":15364,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":15364,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":17052,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2942,"record_offset":16180,"parent_record_offset":16156,"ancestor_record_offset":15364,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2942,"record_offset":16688,"parent_record_offset":16656,"ancestor_record_offset":15364,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2942,"record_offset":15856,"parent_record_offset":15684,"ancestor_record_offset":15364,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":16068,"parent_record_offset":15964,"ancestor_record_offset":15364,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2942,"record_offset":17240,"parent_record_offset":17208,"ancestor_record_offset":15364,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2942,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":15364,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02942.jsonl.gz"}
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
// 首条位置: {"module":2942,"record_offset":11748,"rva":37367888,"va":5406077008}
// 编译实例 RVAs: 0x23a3050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_text::TextFragment::render
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24063734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406077008,"raw_hex":"e80ad190fe","size":5,"soft_fail":false,"target_va":5382013696,"va":5406077425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":15296,"parent_record_offset":11748,"ancestor_record_offset":11748,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":15232,"parent_record_offset":11748,"ancestor_record_offset":11748,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":12312,"parent_record_offset":11748,"ancestor_record_offset":11748,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":11880,"parent_record_offset":11748,"ancestor_record_offset":11748,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11748,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":11748,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2942,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":11748,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":11748,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":12096,"parent_record_offset":12064,"ancestor_record_offset":11748,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":11748,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":11748,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2942,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":11748,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":13444,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":12604,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":12688,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":13696,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2942,"record_offset":12520,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":15324,"parent_record_offset":15296,"ancestor_record_offset":11748,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":15260,"parent_record_offset":15232,"ancestor_record_offset":11748,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13776,"parent_record_offset":13752,"ancestor_record_offset":11748,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":12916,"parent_record_offset":12780,"ancestor_record_offset":11748,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":11748,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13928,"parent_record_offset":13724,"ancestor_record_offset":11748,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":11748,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":11748,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":12828,"parent_record_offset":12804,"ancestor_record_offset":11748,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13752,"parent_record_offset":13724,"ancestor_record_offset":11748,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":11748,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2942,"record_offset":13724,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":12016,"parent_record_offset":11904,"ancestor_record_offset":11748,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11748,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2942,"record_offset":12352,"parent_record_offset":12312,"ancestor_record_offset":11748,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2942,"record_offset":12780,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2942,"record_offset":14616,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2942,"record_offset":13388,"parent_record_offset":13364,"ancestor_record_offset":11748,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":11748,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":12876,"parent_record_offset":12852,"ancestor_record_offset":11748,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2942,"record_offset":13212,"parent_record_offset":13188,"ancestor_record_offset":11748,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":11748,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":11748,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":14532,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":11748,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":14448,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2942,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":11748,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2942,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":11748,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11748,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":11748,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":11904,"parent_record_offset":11880,"ancestor_record_offset":11748,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2942,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11748,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2942,"record_offset":14000,"parent_record_offset":13976,"ancestor_record_offset":11748,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2942,"record_offset":13124,"parent_record_offset":13064,"ancestor_record_offset":11748,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":13148,"parent_record_offset":13124,"ancestor_record_offset":11748,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":13188,"parent_record_offset":13016,"ancestor_record_offset":11748,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2942,"record_offset":12260,"parent_record_offset":12236,"ancestor_record_offset":11748,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2942,"record_offset":12484,"parent_record_offset":12456,"ancestor_record_offset":11748,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":11748,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2942,"record_offset":13640,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2942,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":11748,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2942,"record_offset":13268,"parent_record_offset":13236,"ancestor_record_offset":11748,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2942,"record_offset":12236,"parent_record_offset":12064,"ancestor_record_offset":11748,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2942,"record_offset":12456,"parent_record_offset":12352,"ancestor_record_offset":11748,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2942,"record_offset":13856,"parent_record_offset":13824,"ancestor_record_offset":11748,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2942,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":11748,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02942.jsonl.gz"}
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
// 首条位置: {"module":2942,"record_offset":3792,"rva":37364848,"va":5406073968}
// 编译实例 RVAs: 0x23a2470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7f490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26357842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406073968,"raw_hex":"e8aecf6dfe","size":5,"soft_fail":false,"target_va":5379716240,"va":5406074077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411a3eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18867793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406073968,"raw_hex":"e8af19e0fe","size":5,"soft_fail":false,"target_va":5387206320,"va":5406074108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429b3a80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: text.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6362607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406073968,"raw_hex":"e8ef156100","size":5,"soft_fail":false,"target_va":5412436608,"va":5406073996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10547464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406073968,"raw_hex":"e808f1a000","size":5,"soft_fail":false,"target_va":5416621600,"va":5406074131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5406073968,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5406074013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":4100,"parent_record_offset":3792,"ancestor_record_offset":3792,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2942,"record_offset":3976,"parent_record_offset":3792,"ancestor_record_offset":3792,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2942,"record_offset":3916,"parent_record_offset":3792,"ancestor_record_offset":3792,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2942,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3792,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2942,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3792,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/02942.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::text::flush_text_render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11287
// 首条位置: {"module":1240,"inlinee":70279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1240,"record_offset":5580,"parent_record_offset":5056,"ancestor_record_offset":5056,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1240,"record_offset":5116,"parent_record_offset":5056,"ancestor_record_offset":5056,"inlinee":27015,"source_line":168,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1240,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":5056,"inlinee":9288,"source_line":372,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1240,"record_offset":5304,"parent_record_offset":5144,"ancestor_record_offset":5056,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1240,"record_offset":5192,"parent_record_offset":5144,"ancestor_record_offset":5056,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1240,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":5056,"inlinee":26713,"source_line":215,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::BufferHandleExt::desc
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/buffer.rs
//     evidence={"module":1240,"record_offset":5276,"parent_record_offset":5144,"ancestor_record_offset":5056,"inlinee":26714,"source_line":109,"archive":"_pdb/C/modules/01240.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/bytes.rs
//     evidence={"module":1240,"record_offset":5220,"parent_record_offset":5144,"ancestor_record_offset":5056,"inlinee":9287,"source_line":8,"archive":"_pdb/C/modules/01240.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 23; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
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
// 信息 ID: 30144e252d3e77f83c4a31e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8899861bcffb74c21d1ad7c3: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x11d1805 -> 0x23a2470 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x230d69c -> 0x23a2470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::flush_text_render::closure$0
// 信息 ID: 03353d13faf3b8c14b55bc08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::flush_text_render", "captures": "unknown"}
// [闭包环境类型引用] toolbar::text::flush_text_render::closure_env$0 <- tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0>; module=1240, inline_record=5056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::prepare_text_fragments
// 信息 ID: 83dcce9848d873c17d8c2bf1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8e3d842129617e04da1c3ac2: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> @ 0x11d141c -> 0x23a3300 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<ref_mut$<tg_text::TextFragment>,tuple$<> >)> > @ 0x2304fb4 -> 0x23a3300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::render_text_fragments
// 信息 ID: b25a298de2e40871f25e2819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e826ead3593b9199e32767d: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> @ 0x11d15bc -> 0x23a3050 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)> > @ 0x230ce54 -> 0x23a3050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system
// 信息 ID: c3cd6603ba848ea9d52aa110
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1a12d802edafed7f79fb1a9f: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> @ 0x11d1bc0 -> 0x23a2530 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>)> > @ 0x2311998 -> 0x23a2530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$0
// 信息 ID: d1397496cf8d641711975bba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2942, inline_record=4572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$1
// 信息 ID: 353c1f5b92f6ce11e27e54de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2942, inline_record=5948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::text_startup_system::closure$2
// 信息 ID: 0a28fd1ca8a6ab79a0b7b7e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::text::text_startup_system", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::text::text_startup_system; module=2942, inline_record=6648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::text::update_text_renderer_scale
// 信息 ID: 160c2b0b23f3ea47c966f889
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b102c492741a390ab7ddbd1e: void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> @ 0x11d1a1e -> 0x23a3500 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > @ 0x2307c41 -> 0x23a3500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
